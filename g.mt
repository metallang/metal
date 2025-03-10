abstract Interactable {
    def interact(with: GameObject): void;
}

const OBJECT_POOL_SIZE: usize = 4096;

enum PlayerKind {
    Local
    Remote
    Unknown(String)

    def is_known(self): bool {
        return true;
    }
}

def replace_with_zeroes(mut string: String): void {
    // todo
}

import std.{fs.File, io.{open, close}};

return;

struct GameObject {
    pub id: usize;
    generation: usize;

    def new(): Self {
        // todo
    }
}

type PlayerType = PlayerKind;
type GenericNameType = Array[usize]
type RefType = &mut GenericNameType
type ReallyRefType = &mut &&mut GenericNameType
// type BinaryType = GenericNameType & RefType
