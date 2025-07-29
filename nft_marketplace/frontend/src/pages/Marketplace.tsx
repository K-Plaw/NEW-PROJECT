import { useEffect, useState } from "react";
import "./Marketplace.module.css";

interface NFT {
  id: number;
  title: string;
  creator: string;
  image: string;
  price: string;
}

const mockNFTs: NFT[] = [
  {
    id: 1,
    title: "Cyber Lion",
    creator: "ZeeBee Arts",
    image: "https://via.placeholder.com/300x300.png?text=Cyber+Lion",
    price: "2.5 SOL",
  },
  {
    id: 2,
    title: "Digital Phoenix",
    creator: "CreativeMe",
    image: "https://via.placeholder.com/300x300.png?text=Digital+Phoenix",
    price: "3.2 SOL",
  },
  {
    id: 3,
    title: "Neon Skull",
    creator: "GhostMint",
    image: "https://via.placeholder.com/300x300.png?text=Neon+Skull",
    price: "1.8 SOL",
  },
  {
    id: 4,
    title: "Futuristic Ape",
    creator: "ZeeBee Arts",
    image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSTrIXEuQteCrpc-zQYqGG9Ty4q-_qUUd8d2Q&s",
    price: "4.0 SOL",
  },
];

const MarketPlace = () => {
  const [nfts, setNfts] = useState<NFT[]>([]);

  useEffect(() => {
    // Simulate API call
    setTimeout(() => {
      setNfts(mockNFTs);
    }, 500);
  }, []);

  return (
    <div className="marketplace-container">
      <h1 className="marketplace-heading">🛍️ Explore Marketplace</h1>

      <div className="nft-grid">
        {nfts.map((nft) => (
          <div key={nft.id} className="nft-card">
            <img src={nft.image} alt={nft.title} className="nft-image" />
            <h2 className="nft-title">{nft.title}</h2>
            <p className="nft-creator">by {nft.creator}</p>
            <p className="nft-price">{nft.price}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default MarketPlace;
