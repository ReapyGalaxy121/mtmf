// Cadence 1.0 NFT Minting Transaction
// Compatible with Flow Crescendo upgrade (Sept 2024)

import NonFungibleToken from 0x1d7e57aa55817448
import MetadataViews from 0x1d7e57aa55817448
import ViewResolver from 0x1d7e57aa55817448

transaction(
    recipient: Address,
    name: String,
    description: String,
    thumbnail: String,
    ipfsHash: String
) {
    
    let minter: auth(NonFungibleToken.Mint) &{NonFungibleToken.Collection}
    let recipientCollectionRef: &{NonFungibleToken.CollectionPublic}

    prepare(signer: auth(BorrowValue) &Account) {
        // Borrow the minter reference from the stored collection
        self.minter = signer.storage.borrow<auth(NonFungibleToken.Mint) &{NonFungibleToken.Collection}>(
            from: /storage/NFTCollection
        ) ?? panic("Could not borrow minter reference")

        // Get the recipient's public account object
        let recipientAccount = getAccount(recipient)

        // Borrow the recipient's public NFT collection reference
        self.recipientCollectionRef = recipientAccount.capabilities
            .borrow<&{NonFungibleToken.CollectionPublic}>(/public/NFTCollection)
            ?? panic("Could not borrow recipient collection reference")
    }

    execute {
        // Create the NFT metadata
        let metadata: {String: AnyStruct} = {}
        metadata["name"] = name
        metadata["description"] = description
        metadata["thumbnail"] = thumbnail
        metadata["ipfsHash"] = ipfsHash

        // Mint the NFT
        let nft <- self.minter.mint(metadata: metadata)

        // Deposit the NFT in the recipient's collection
        self.recipientCollectionRef.deposit(token: <-nft)

        log("NFT minted and deposited to recipient")
    }
}
