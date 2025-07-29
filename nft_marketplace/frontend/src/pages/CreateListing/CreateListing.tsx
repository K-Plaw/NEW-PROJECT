import { useState } from "react";
import styles from "./CreateListing.module.css";

const CreateListing = () => {
  const [nftAddress, setNftAddress] = useState("");
  const [price, setPrice] = useState("");
  const [royalty, setRoyalty] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Listing Data:", { nftAddress, price, royalty });
  };

  return (
    <div className={styles.container}>
      <h2 className={styles.heading}>Create New Listing</h2>
      <form onSubmit={handleSubmit} className={styles.form}>
        <div>
          <label className={styles.label}>NFT Address</label>
          <input
            type="text"
            value={nftAddress}
            onChange={(e) => setNftAddress(e.target.value)}
            className={styles.input}
            placeholder="Enter NFT address"
            required
          />
        </div>
        <div>
          <label className={styles.label}>Price (in SOL)</label>
          <input
            type="number"
            value={price}
            onChange={(e) => setPrice(e.target.value)}
            className={styles.input}
            placeholder="e.g. 1.5"
            step="0.01"
            required
          />
        </div>
        <div>
          <label className={styles.label}>Creator Royalty (%)</label>
          <input
            type="number"
            value={royalty}
            onChange={(e) => setRoyalty(e.target.value)}
            className={styles.input}
            placeholder="e.g. 10"
            min="0"
            max="100"
          />
        </div>
        <button type="submit" className={styles.button}>
          List NFT
        </button>
      </form>
    </div>
  );
};

export default CreateListing;
