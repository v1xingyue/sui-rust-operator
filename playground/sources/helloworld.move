module playground::hello_world {

    use std::string;
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    /// An object that contains an arbitrary string
    struct HelloWorldObject has key, store {
        id: UID,
        /// A string contained in the object
        text: string::String
    }

    entry fun mint(ctx: &mut TxContext) {
        let object = HelloWorldObject {
            id: object::new(ctx),
            text: string::utf8(b"Hello World!")
        };
        transfer::public_transfer(object, tx_context::sender(ctx));
    }

    entry fun mint_to(to_addr:address,ctx: &mut TxContext) {
        let object = HelloWorldObject {
            id: object::new(ctx),
            text: string::utf8(b"Hello World!")
        };
        transfer::public_transfer(object, to_addr);
    }

    entry fun update_text(obj: &mut HelloWorldObject,new_text: string::String){
        obj.text = new_text;
    }

    entry fun destroy(obj:HelloWorldObject){
        let HelloWorldObject{id,text:_} = obj;
        object::delete(id);
    }
}