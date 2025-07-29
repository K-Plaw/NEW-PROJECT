import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Marketplace from "./pages/Marketplace";
import CreateListing from "./pages/CreateListing";
import MyListings from "./pages/MyListings";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";
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
