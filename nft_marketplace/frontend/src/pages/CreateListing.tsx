import { useState } from "react";

const CreateListing = () => {
  const [nftAddress, setNftAddress] = useState("");
  const [price, setPrice] = useState("");
  const [royalty, setRoyalty] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Listing Data:", { nftAddress, price, royalty });
    // We'll replace this with actual logic to call the Anchor backend later
  };

  return (
    <div className="max-w-2xl mx-auto p-6 bg-white dark:bg-gray-900 rounded-2xl shadow-xl mt-10">
      <h2 className="text-2xl font-bold text-blue-600 mb-6">Create New Listing</h2>
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
            NFT Address
          </label>
          <input
            type="text"
            value={nftAddress}
            onChange={(e) => setNftAddress(e.target.value)}
            className="w-full p-3 mt-1 border rounded-xl bg-gray-100 dark:bg-gray-800 dark:text-white"
            placeholder="Enter NFT address"
            required
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
            Price (in SOL)
          </label>
          <input
            type="number"
            value={price}
            onChange={(e) => setPrice(e.target.value)}
            className="w-full p-3 mt-1 border rounded-xl bg-gray-100 dark:bg-gray-800 dark:text-white"
            placeholder="e.g. 1.5"
            step="0.01"
            required
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-200">
            Creator Royalty (%)
          </label>
          <input
            type="number"
            value={royalty}
            onChange={(e) => setRoyalty(e.target.value)}
            className="w-full p-3 mt-1 border rounded-xl bg-gray-100 dark:bg-gray-800 dark:text-white"
            placeholder="e.g. 10"
            min="0"
            max="100"
          />
        </div>
        <button
          type="submit"
          className="w-full bg-blue-600 text-white py-3 rounded-xl hover:bg-blue-700 transition"
        >
          List NFT
        </button>
      </form>
    </div>
  );
};

export default CreateListing;
