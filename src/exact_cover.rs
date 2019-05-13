struct DataObject {
    left: Rc<DataObject>,
    right: Rc<DataObject>,
    up: Rc<DataObject>,
    down: Rc<DataObject>,
    header: Rc<ColumnObject>,
}

struct ColumnObject {
    data: Rc<DataObject>,
    size: usize,
    name: String,
}

struct ExactCover {
    root: ColumnObject
}

impl ExactCover {
}
