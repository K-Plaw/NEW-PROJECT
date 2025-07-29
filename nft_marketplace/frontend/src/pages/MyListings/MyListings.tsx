import React from 'react';
import styles from './MyListings.module.css';

interface Listing {
  id: number;
  name: string;
  // Extend with more NFT fields when needed
}

const MyListings: React.FC = () => {
  const listings: Listing[] = []; // Placeholder for user-owned NFTs

  return (
    <div className={styles.container}>
      <h1 className={styles.heading}>My Listings</h1>

      {listings.length === 0 ? (
        <p className={styles.emptyText}>You haven't listed any NFTs yet.</p>
      ) : (
        <div className={styles.grid}>
          {listings.map((_, index) => (
            <div key={index} className={styles.card}>
              <p>NFT #{index + 1}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default MyListings;
