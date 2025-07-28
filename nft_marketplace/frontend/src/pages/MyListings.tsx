import React from 'react';

interface Listing {
  id: number;
  name: string;
  // Add more fields later based on actual NFT structure
}

const MyListings: React.FC = () => {
  const listings: Listing[] = []; // explicitly typed

  return (
    <div className="min-h-screen p-6 bg-gradient-to-b from-blue-50 to-white text-gray-800">
      <h1 className="text-3xl font-bold mb-6">My Listings</h1>
      {listings.length === 0 ? (
        <p className="text-lg text-gray-500">You haven't listed any NFTs yet.</p>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
          {listings.map((_, index) => (
            <div key={index} className="bg-white shadow-md rounded-xl p-4">
              <p>NFT #{index + 1}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default MyListings;
