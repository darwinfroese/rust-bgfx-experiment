pub trait Scene {
    fn get_id(self: &Self) -> u16;

    fn transition_in(self: &Self);
    fn transition_out(self: &Self);

    fn update(self: &Self);
}
