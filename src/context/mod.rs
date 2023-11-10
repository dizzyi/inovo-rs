use std::ops::{Deref, DerefMut};

use crate::logger::Logable;

/// A trait for context machine
///
/// context machine have the ability to enter and exit contexts in a stack,
///
/// there are routine associate with both entering and exiting a certain context.
///
/// # Exmple
/// ```ignore
/// fn main(){
///     let mut context_machine = MyContextMachine::new();
///     context_machine.some_fun();
///     // ...
///     {
///             
///         let guard = context_machine.with(Box::new(MyContext::new()));
///         //                          ^ when the context spawn, it will routine a enter ritual
///         // ...
///         // then you can use the guard as the original context machine
///         context_machine.some_fun();
///         // ...
///     }// <--- when the guard is drop, the exit routine will be performed
/// }
/// ```
/// ## stacking context
/// if you need to stack context with only one function call,
///
/// you can stack them and call directly
/// ```ignore
/// context_machine
///     .with(Box::new(MyContext1::new()))
///     .with(Box::new(MyContext2::new())) // <-- you can stack the context
///     .some_fun();                       // <-- and use the result as original
/// ```
/// ## stacking context in block
/// if you need to stack context with multiple statment
///
/// you need to `let` guard individually, it will drop at the end of scope
///
/// or you can drop it manually.
/// ```ignore
/// {
///     let mut guard1 = context_machine.with(Box::new(MyContext1::new()));
///     let mut guard2 = guard.with(Box::new(MyContext2::new()));
///
///     guard2.some_fun();
///     guard2.some_fun2();
///
///     drop(guard2) // <--- guard2 is dropped, guard1 is usable again
///
///     guard1.some_fun();
/// }// <--- guard1 is dropped
/// ```
pub trait ContextMachine: Sized + Logable {
    /// get the stack of the context
    fn context_stack(&mut self) -> &mut Vec<Box<dyn Context<Self>>>;
    /// enter a context
    fn context_enter(&mut self, mut context: Box<dyn Context<Self>>) -> Result<(), String> {
        self.debug(format!(">>> Entering Context {}", context.label()))?;
        context.enter_fn(self)?;
        self.context_stack().push(context);
        Ok(())
    }
    /// exit a context
    fn context_exit(&mut self) -> Result<(), String> {
        let mut context = self
            .context_stack()
            .pop()
            .ok_or(format!("No Context to Exit"))?;
        self.debug(format!("<<< Exiting Context {}", context.label()))?;
        context.exit_fn(self)
    }
    /// enter a context and return the guard
    fn with(&mut self, context: Box<dyn Context<Self>>) -> Result<ContextGuard<Self>, String> {
        self.context_enter(context)?;
        Ok(ContextGuard::new(self))
    }
}

/// A trait shared by context that is usable by `ContextMachine`
pub trait Context<T>
where
    T: ContextMachine,
{
    /// function called when entering context
    fn enter_fn(&mut self, t: &mut T) -> Result<(), String>;
    /// function called when exiting context
    fn exit_fn(&mut self, t: &mut T) -> Result<(), String>;
    /// label of the context
    fn label(&self) -> String;
}

/// A guard for context management
///
/// when it is dropped, it perform the exit function of the context
pub struct ContextGuard<'a, M>
where
    M: ContextMachine,
{
    m: &'a mut M,
}

impl<'a, M> ContextGuard<'a, M>
where
    M: ContextMachine,
{
    /// create a new guard
    pub fn new(m: &'a mut M) -> Self {
        Self { m }
    }
    /// exit the context
    pub fn exit(&mut self) -> Result<(), String> {
        self.m.context_exit()
    }
}

impl<'a, M> Deref for ContextGuard<'a, M>
where
    M: ContextMachine,
{
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

impl<'a, M> DerefMut for ContextGuard<'a, M>
where
    M: ContextMachine,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m
    }
}

impl<'a, M> Drop for ContextGuard<'a, M>
where
    M: ContextMachine,
{
    fn drop(&mut self) {
        let _ = self.exit();
    }
}
