import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Marketplace from "./pages/Marketplace";
import CreateListing from "./pages/CreateListing";
import MyListings from "./pages/MyListings";
import Navbar from "./components/Navbar";
import Footer from "./components/Footer";

export default function App() {
  return (
    <Router>
      <div className="bg-[#f4faff] min-h-screen text-gray-900">
        <Navbar />
        <main className="pt-20 px-4 md:px-12">
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
