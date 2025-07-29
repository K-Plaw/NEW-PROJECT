// src/components/Footer.tsx
import React from "react";
import styles from "./Footer.module.css";
import logo from "../../assets/logo.png"; 
import twitterIcon from "../../assets/Socials/X.jpeg"; 
import githubIcon from "../../assets/Socials/FB.jpeg";
import discordIcon from "../../assets/Socials/INSTAGRAM.jpeg";

const Footer: React.FC = () => {
  return (
    <footer className={styles.footer}>
      <div className={styles.container}>
        <div className={styles.brand}>
          <img src={logo} alt="Anchor NFT" className={styles.logo} />
          <p>&copy; {new Date().getFullYear()} Anchor NFT Marketplace</p>
        </div>

        <div className={styles.links}>
          <a href="#">Terms</a>
          <a href="#">Privacy</a>
          <a href="#">Contact</a>
        </div>

        <div className={styles.socials}>
          <a href="https://twitter.com" target="_blank" rel="noopener noreferrer">
            <img src={twitterIcon} alt="Twitter" />
          </a>
          <a href="https://github.com" target="_blank" rel="noopener noreferrer">
            <img src={githubIcon} alt="GitHub" />
          </a>
          <a href="https://discord.com" target="_blank" rel="noopener noreferrer">
            <img src={discordIcon} alt="Discord" />
          </a>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
