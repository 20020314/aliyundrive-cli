use serde::ser;

pub trait RW<T1, T2>
where
    T1: ser::Serialize,
    T2: ser::Serialize
{
    fn write(t: T1) -> serde_yaml::Result<()>;

    fn read() -> serde_yaml::Result<T1>;

    fn read_token(is_mobile: bool) -> serde_yaml::Result<T2>;

    fn write_token(is_mobile: bool, t: T2) -> serde_yaml::Result<()>;
}
