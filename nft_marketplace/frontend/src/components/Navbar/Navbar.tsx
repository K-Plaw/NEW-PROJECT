import { Link } from "react-router-dom";
import { useState } from "react";
import { Menu, X } from "lucide-react";
import styles from "./Navbar.module.css";
import logo from "../../assets/logo.png";

const Navbar = () => {
  const [isOpen, setIsOpen] = useState(false);

  const toggleMenu = () => setIsOpen((prev) => !prev);

  return (
    <nav className={styles.navbar}>
      <div className={styles.container}>
        <Link to="/" className={styles.logoContainer}>
          <img src={logo} alt="NFT Market Logo" className={styles.logoImage} />
          <span className={styles.logoText}>NFT Market</span>
        </Link>

        <ul className={styles.menu}>
          <li><Link to="/explore" className={styles.link}>Explore</Link></li>
          <li><Link to="/create" className={styles.link}>Create</Link></li>
          <li><Link to="/my-nfts" className={styles.link}>My NFTs</Link></li>
        </ul>

        <button onClick={toggleMenu} className={styles.toggleButton}>
          {isOpen ? <X size={24} /> : <Menu size={24} />}
        </button>
      </div>

      {isOpen && (
        <div className={styles.mobileMenu}>
          <ul>
            <li><Link to="/explore" onClick={toggleMenu} className={styles.mobileLink}>Explore</Link></li>
            <li><Link to="/create" onClick={toggleMenu} className={styles.mobileLink}>Create</Link></li>
            <li><Link to="/my-nfts" onClick={toggleMenu} className={styles.mobileLink}>My NFTs</Link></li>
          </ul>
        </div>
      )}
    </nav>
  );
};

export default Navbar;
