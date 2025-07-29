import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home/Home";
import Marketplace from "./pages/MarketPlace/Marketplace";
import CreateListing from "./pages/CreateListing/CreateListing";
import MyListings from "./pages/MyListings/MyListings";
import Navbar from "./components/Navbar/Navbar";
import Footer from "./components/Footer/Footer";
import styles from "./App.module.css";

export default function App() {
  return (
    <Router>
      <div className={styles.appContainer}>
        <Navbar />
        <main className={styles.main}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/marketplace" element={<Marketplace />} />
            <Route path="/create" element={<CreateListing />} />
            <Route path="/my-listings" element={<MyListings />} />
          </Routes>
        </main>
        <Footer />
      </div>
    </Router>
  );
}
