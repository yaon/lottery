

trait Function {
    fn start(&self, Sender<T>, Receiver<T>) -> Function;
    fn exit() -> ();
}

