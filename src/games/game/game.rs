use maud::Markup;

pub trait Game {
    fn render(&self) -> Markup;
}
