pub mod tracks;
pub mod albums;
pub mod artists;
pub mod artist;

pub trait GroupScrobble<'a> {
    type Result;
    type Source;
    type Include;

    fn group(source: Self::Source, include: Self::Include) -> Self::Result;
}
