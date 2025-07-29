// src/components/NFTCard.tsx
import React from 'react';
import styles from './NFTCard.module.css';

interface NFTCardProps {
  id: number;
  title: string;
  creator: string;
  price: string;
  image: string;
  description: string;
  isExpanded: boolean;
  toggleDetails: (id: number) => void;
}

const NFTCard: React.FC<NFTCardProps> = ({
  id,
  title,
  creator,
  price,
  image,
  description,
  isExpanded,
  toggleDetails,
}) => {
  return (
    <div className={styles['nft-card']}>
      <img src={image} alt={title} className={styles['nft-image']} />
      <h2 className={styles['nft-title']}>{title}</h2>
      <p className={styles['nft-creator']}>Creator: {creator}</p>
      <p className={styles['nft-price']}>{price}</p>

      {!isExpanded ? (
        <button
          onClick={() => toggleDetails(id)}
          className={styles['view-details-btn']}
        >
          View Details
        </button>
      ) : (
        <>
          <p className={styles['nft-description']}>{description}</p>
          <div className={styles['nft-actions']}>
            <button className={styles['buy-now-btn']}>Buy Now</button>
            <button
              onClick={() => toggleDetails(id)}
              className={styles['hide-details-btn']}
            >
              Hide Details
            </button>
          </div>
        </>
      )}
    </div>
  );
};

export default NFTCard;
