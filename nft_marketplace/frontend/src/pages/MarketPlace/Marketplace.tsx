// src/pages/Marketplace.tsx
import React, { useState } from 'react';
import nftSample from '../../assets/sample-nft.jpg';
import styles from './Marketplace.module.css';
import NFTCard from '../../components/NFTCard/NFTCard';

interface NFT {
  id: number;
  title: string;
  creator: string;
  price: string;
  image: string;
  description: string;
}

const sampleNFTs: NFT[] = [
  {
    id: 1,
    title: 'Ocean Bloom',
    creator: '0xA1B2C3',
    price: '2.1 SOL',
    image: nftSample,
    description: 'A mesmerizing bloom over calm blue waters.',
  },
  {
    id: 2,
    title: 'Pixel Pirate',
    creator: '0xD4E5F6',
    price: '4.3 SOL',
    image: nftSample,
    description: 'Retro-style pirate NFT with pixelated flair.',
  },
  // Add more NFTs here
];

const Marketplace: React.FC = () => {
  const [expandedCards, setExpandedCards] = useState<number[]>([]);

  const toggleDetails = (id: number) => {
    setExpandedCards(prev =>
      prev.includes(id) ? prev.filter(i => i !== id) : [...prev, id]
    );
  };

  return (
    <div className={styles['marketplace-container']}>
      <h1 className={styles['marketplace-heading']}>Explore Our NFTs</h1>
      <div className={styles['nft-grid']}>
        {sampleNFTs.map(nft => (
          <NFTCard
            key={nft.id}
            {...nft}
            isExpanded={expandedCards.includes(nft.id)}
            toggleDetails={toggleDetails}
          />
        ))}
      </div>
    </div>
  );
};

export default Marketplace;
