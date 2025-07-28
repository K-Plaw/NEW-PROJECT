import { useEffect, useState } from "react";

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
    image: "https://via.placeholder.com/300x300.png?text=Futuristic+Ape",
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
    <div className="pt-24 px-6 md:px-12">
      <h1 className="text-3xl font-bold text-blue-900 mb-8">🛍️ Explore Marketplace</h1>

      <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-8">
        {nfts.map((nft) => (
          <div key={nft.id} className="bg-white rounded-xl shadow-md hover:shadow-xl transition p-4">
            <img
              src={nft.image}
              alt={nft.title}
              className="w-full h-48 object-cover rounded-lg mb-4"
            />
            <h2 className="text-xl font-semibold text-blue-900">{nft.title}</h2>
            <p className="text-sm text-blue-600 mb-2">by {nft.creator}</p>
            <p className="text-lg font-bold text-blue-700">{nft.price}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default MarketPlace;
