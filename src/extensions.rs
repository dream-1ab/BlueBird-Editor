/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-10 06:35:57
 * @modify date 2024-09-10 06:35:57
 * @desc [description]
*/

struct MyExts {

}

pub trait AnyExts<F> {
    fn let_self(self, applier: F) -> Self;
}


impl<T, F: FnOnce(Self) -> Self> AnyExts<F> for T {
    fn let_self(self, applier: F) -> Self {
        let mut me = self;
        me = applier(me);
        me
    }
}
