use std::{
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    rc::Rc,
};

fn main() {
    let a: i32 = 5;
    let b: &i32 = &a;
    assert_eq!(*b, 5);

    let mut b: Box<i32> = Box::new(6);
    *b = 7;
    assert_eq!(*b, 7);

    let mut b: Box<i32> = Box::new(6);

    let c: &mut i32 = b.deref_mut();
    *c = 7;

    let d: &i32 = b.deref();
    assert_eq!(*d, 7);

    // implicitly
    let rc1 = Rc::new(3);
    assert_eq!(*rc1, 3);
    // explicitly
    let a = rc1.deref();
    assert_eq!(*a, 3);

    let ref_cell = RefCell::new(5);
    *ref_cell.borrow_mut() = 7;
    assert_eq!(*ref_cell.borrow(), 7);

    let ref_cell = RefCell::new(5);
    *(ref_cell.borrow_mut().deref_mut()) = 7;
    assert_eq!(*(ref_cell.borrow().deref()), 7);


    let ref_cell = RefCell::new(5);
    {
        let mut a: RefMut<i32> = ref_cell.borrow_mut();
        let b: &mut i32 = a.deref_mut();
        *b = 8;
    } // borrow_mut will be drop here, at the end of block
    {
        // there are no mutability reference to the value, so we can borrow.
        let a: Ref<i32> = ref_cell.borrow();
        let b: &i32 = a.deref();
        assert_eq!(*b, 8);
    }

    let ref_cell2 = RefCell::new(Some(5));
    if let Some(ref mut a) = *ref_cell2.borrow_mut() {
        *a = 6;
    }
    assert_eq!(*ref_cell2.borrow(), Some(6));
    match ref_cell2.borrow_mut().deref_mut() {
        Some(val) => {
            *val = 8;
        }
        None => {}
    }
    assert_eq!(*ref_cell2.borrow(), Some(8));

    match ref_cell2.borrow_mut().deref_mut().as_mut() {
        Some(val) => {}
        None => {}
    }

    let rc = Rc::new(RefCell::new(5));
    *rc.borrow_mut() = 6;
    assert_eq!(*rc.borrow(), 6);
}
