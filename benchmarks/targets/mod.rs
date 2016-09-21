pub trait Backend {
    fn putter(&mut self) -> Box<Putter>;
    fn getter(&mut self) -> Box<Getter>;
}

pub trait Putter: Send {
    fn article<'a>(&'a mut self) -> Box<FnMut(i64, String) + 'a>;
    fn vote<'a>(&'a mut self) -> Box<FnMut(i64, i64) + 'a>;
}

pub trait Getter: Send {
    fn get<'a>(&'a self) -> Box<FnMut(i64) -> Option<(i64, String, i64)> + 'a>;
}

pub mod soup;
#[cfg(feature="b_postgresql")]
pub mod postgres;
#[cfg(feature="b_netsoup")]
pub mod netsoup;
#[cfg(feature="b_memcached")]
pub mod memcached;
