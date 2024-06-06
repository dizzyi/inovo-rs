use inovo_rs::context::*;
use inovo_rs::logger::*;

struct ContextMachine {
    pub logger: Logger,
}

impl ContextMachine {
    pub fn doing_stuff(&mut self) {
        self.logger.info("doing stuff");
    }

    pub fn with_context_1(&mut self) -> ContextGuard<Self, Context1> {
        ContextGuard::new(self, Context1)
    }
    pub fn with_context_2(&mut self) -> ContextGuard<Self, Context2> {
        ContextGuard::new(self, Context2)
    }

    pub fn start_up_1(&mut self) {
        self.logger.info("starting up 1");
    }
    pub fn start_up_2(&mut self) {
        self.logger.info("starting up 2");
    }
    pub fn clean_up_1(&mut self) {
        self.logger.info("clean up 1");
    }
    pub fn clean_up_2(&mut self) {
        self.logger.info("clean up 2");
    }
}

pub struct Context1;

impl Context<ContextMachine> for Context1 {
    fn context_enter(&mut self, machine: &mut ContextMachine) {
        machine.start_up_1()
    }
    fn context_drop(&mut self, machine: &mut ContextMachine) {
        machine.clean_up_1()
    }
}

pub struct Context2;

impl Context<ContextMachine> for Context2 {
    fn context_enter(&mut self, machine: &mut ContextMachine) {
        machine.start_up_2()
    }
    fn context_drop(&mut self, machine: &mut ContextMachine) {
        machine.clean_up_2()
    }
}

fn do_some_stuff() {}

fn main() {
    let mut context_machine = ContextMachine {
        logger: Logger::default_target("context machine"),
    };

    // Simple usage
    //
    // for a task that need to be start up and clean up
    // this is how it need to be
    context_machine.start_up_1();
    context_machine.doing_stuff();
    context_machine.clean_up_1();
    //
    // but with the context structure
    // we can defer the clean up function
    // let rust's ownership rule determinte when to clean up
    context_machine.with_context_1().doing_stuff();

    // Chaining usage
    //
    // to achieve chained usage like this
    context_machine.start_up_1();
    context_machine.start_up_2();
    context_machine.doing_stuff();
    context_machine.clean_up_2();
    context_machine.clean_up_1();
    //
    // we can chain the context together
    context_machine
        .with_context_1()
        .with_context_2()
        .doing_stuff();

    // Scope usage
    //
    // if there are other operation need to done within the context
    // like this
    context_machine.start_up_1();
    do_some_stuff();
    context_machine.doing_stuff();
    do_some_stuff();
    context_machine.clean_up_1();
    //
    // we can scope the guard
    {
        let mut guard = context_machine.with_context_1();
        do_some_stuff();
        guard.doing_stuff();
        do_some_stuff();
    }

    // Chained Scope usage
    //
    // to use multiple context in a process
    // like this
    context_machine.start_up_1();
    context_machine.start_up_2();
    do_some_stuff();
    context_machine.doing_stuff();
    do_some_stuff();
    context_machine.clean_up_2();
    context_machine.clean_up_1();
    //
    // we can chain the context
    {
        let mut guard_1 = context_machine.with_context_1();
        let mut guard_2 = guard_1.with_context_2();
        do_some_stuff();
        guard_2.doing_stuff();
        do_some_stuff();
        // you can early drop the guard with drop()
        // drop(guard_2);
        // guard_1.doing_stuff();
    }
}
