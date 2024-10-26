
/// Call .draw(context) on all given objects :)
macro_rules! draw_vec {
    ($who:ident, $what:ident, $context:ident) => {
        $who.$what.iter().for_each(|object| {object.draw($context)});
    };
}

macro_rules! update_vec {
	// TODO: Make this able to take as many args as we could want
    ($who:ident, $what:ident, $arg:ident) => {
        $who.$what.iter_mut().for_each(|object| {object.update($arg)});
    };
}
