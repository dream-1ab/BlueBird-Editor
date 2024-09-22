/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 20:28:13
 * @modify date 2024-09-08 20:28:13
 * @desc [description]
*/


type Handler<OWNER, T> = Box<dyn FnMut(&OWNER, &T) -> ()>;
pub struct MessageBus<T, OWNER> {
    id: usize,
    handlers: Vec<(usize, Handler<OWNER, T>)>
}

impl<T, OWNER> MessageBus<T, OWNER> {
    pub fn new() -> Self {
        Self {handlers: vec![], id: 0}
    }

    pub fn subscribe(&mut self, handler: Handler<OWNER, T>) {
        let id = self.id;
        self.id += 1;
        self.handlers.push((id, handler));
    }

    pub fn publish(&mut self, owner: &OWNER, message: &T) {
        self.handlers.iter_mut().for_each(|(id, handler)| {
            handler(owner, message);
        });
    }

    pub fn unsubscribe(&mut self, id: usize) -> Result<Handler<OWNER, T>, String> {
        let mut index = 0usize;
        let existing = self.handlers.iter_mut().map(|item| {
            let i = index;
            index += 1;
            (i, item)
        }).find(|(index, item)| *index == id);
        let mut id = 0usize;
        let mut contains = false;
        if let Some(item) = existing {
            id = item.0;
            contains = true;
        }
        if contains {
            let (id, handler) = self.handlers.remove(id as usize);
            return Ok(handler);
        }
        return Err(format!("Handler not found for id {}", id));;
    }
}
