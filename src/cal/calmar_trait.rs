pub trait CalendarDataType {
    fn name(&self) -> String;
    fn priority(&self) -> u8;
}
