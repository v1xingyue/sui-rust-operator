module playground::my_nft {
    use sui::tx_context::{sender, TxContext};
    use std::string::{utf8, String};
    use sui::transfer::{Self, public_transfer};
    use sui::object::{Self, UID};
    use sui::package;
    use sui::display;

    struct MyNFT has key, store {
        id: UID,
        name: String,
        image_url: String
    }

    /// One-Time-Witness for the module.
    struct MY_NFT has drop {}

    fun init(otw: MY_NFT, ctx: &mut TxContext) {

        // Claim the `Publisher` for the package!
        let publisher = package::claim(otw, ctx);

        let display = display::new_with_fields<MyNFT>(
            &publisher,
            vector[utf8(b"name"),utf8(b"image_url")],
            vector[utf8(b"{name}"),utf8(b"{image_url}")],
            ctx
        );

        // Commit first version of `Display` to apply changes.
        display::update_version(&mut display);

        transfer::public_transfer(publisher, sender(ctx));
        transfer::public_transfer(display, sender(ctx));
    }

    public entry fun mint(name:vector<u8>,image_url:vector<u8>,ctx: &mut TxContext) {
        let nft = MyNFT {
            id: object::new(ctx),
            name:utf8(name),
            image_url:utf8(image_url)
        };
        public_transfer(nft, sender(ctx));
    }

}