// src/components/Footer.tsx
import React from 'react';

const Footer: React.FC = () => {
  return (
    <footer className="bg-blue-900 text-white py-6 mt-12">
      <div className="container mx-auto text-center px-4">
        <p className="text-sm">
          &copy; {new Date().getFullYear()} Anchor NFT Marketplace. All rights reserved.
        </p>
        <div className="mt-2">
          <a href="#" className="text-blue-300 hover:underline mx-2">
            Terms
          </a>
          |
          <a href="#" className="text-blue-300 hover:underline mx-2">
            Privacy
          </a>
          |
          <a href="#" className="text-blue-300 hover:underline mx-2">
            Contact
          </a>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
