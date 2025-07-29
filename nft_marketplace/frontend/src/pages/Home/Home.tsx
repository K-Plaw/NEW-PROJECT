import { Link } from "react-router-dom";
import styles from "./Home.module.css";

const Home = () => {
  return (
    <div>
      {/* Hero Section */}
      <section className={styles.hero}>
        <h1 className={styles.heroTitle}>Discover, Collect, and Sell Unique NFTs</h1>
        <p className={styles.heroSubtitle}>
          Join the next-gen digital marketplace powered by Web3.
        </p>
        <Link to="/marketplace" className={styles.exploreBtn}>
          Explore Marketplace
        </Link>
      </section>

      {/* Trending NFTs */}
      <section className={styles.trendingSection}>
        <h2 className={styles.trendingTitle}>🔥 Trending NFTs</h2>
        <div className={styles.cardGrid}>
          {Array(4).fill(0).map((_, i) => (
            <div key={i} className={styles.card}>
              <div className={styles.imagePlaceholder}></div>
              <h3 className={styles.cardTitle}>NFT Title</h3>
              <p className={styles.cardCreator}>Creator Name</p>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
};

export default Home;
