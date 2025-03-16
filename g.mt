pub abstract Interactable {
    def interact(with: GameObject): void;
}

pub const OBJECT_POOL_SIZE: usize = fibbonacci(pow(2, 4), CONSTANTS.FIB_COEFF);

pub enum PlayerKind {
    Local
    Remote
    Unknown(String)

    pub def is_known(self): bool {
        return true;
    }
}

pub def replace_with_zeroes(mut string: String): void {
    // todo
}

pub import std.{fs.File, io.{open, close}};

pub return;

pub struct GameObject {
    pub id: usize;
    generation: usize;

    pub def new(): Self {
        // todo
    }
}

pub type PlayerType = PlayerKind;
type GenericNameType = Array[usize]
type RefType = &mut GenericNameType
type ReallyRefType = &mut &&mut GenericNameType
type BinaryType = GenericNameType & RefType & SomeOtherTye

pub a.b.c()
