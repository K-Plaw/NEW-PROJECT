import { Link } from "react-router-dom";
import { useState } from "react";
import { Menu, X } from "lucide-react";

const Navbar = () => {
  const [isOpen, setIsOpen] = useState(false);

  const toggleMenu = () => setIsOpen((prev) => !prev);

  return (
    <nav className="bg-blue-900 text-white p-4 shadow-md fixed top-0 left-0 w-full z-50">
      <div className="container mx-auto flex justify-between items-center">
        {/* Logo */}
        <Link to="/" className="text-2xl font-bold text-blue-300">
          NFTMarket
        </Link>

        {/* Desktop Menu */}
        <ul className="hidden md:flex space-x-8 text-sm font-medium">
          <li><Link to="/explore" className="hover:text-blue-400">Explore</Link></li>
          <li><Link to="/create" className="hover:text-blue-400">Create</Link></li>
          <li><Link to="/my-nfts" className="hover:text-blue-400">My NFTs</Link></li>
        </ul>

        {/* Mobile Menu Button */}
        <button onClick={toggleMenu} className="md:hidden">
          {isOpen ? <X size={24} /> : <Menu size={24} />}
        </button>
      </div>

      {/* Mobile Dropdown */}
      {isOpen && (
        <div className="md:hidden bg-blue-800 text-white px-4 py-2">
          <ul className="space-y-2 text-sm">
            <li><Link to="/explore" onClick={toggleMenu} className="block hover:text-blue-300">Explore</Link></li>
            <li><Link to="/create" onClick={toggleMenu} className="block hover:text-blue-300">Create</Link></li>
            <li><Link to="/my-nfts" onClick={toggleMenu} className="block hover:text-blue-300">My NFTs</Link></li>
          </ul>
        </div>
      )}
    </nav>
  );
};

export default Navbar;
