import { Link } from "react-router-dom";

const Home = () => {
  return (
    <div className="pt-20">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-blue-900 to-blue-700 text-white py-16 px-6 md:px-12">
        <div className="max-w-5xl mx-auto text-center">
          <h1 className="text-4xl md:text-6xl font-bold mb-4">Discover, Collect, and Sell Unique NFTs</h1>
          <p className="text-lg md:text-xl mb-8 text-blue-100">
            Join the next-gen digital marketplace powered by Web3.
          </p>
          <Link
            to="/explore"
            className="bg-blue-400 hover:bg-blue-300 text-blue-900 font-semibold py-3 px-6 rounded-full transition duration-300"
          >
            Explore Marketplace
          </Link>
        </div>
      </section>

      {/* Trending NFTs (Placeholder) */}
      <section className="bg-white py-12 px-6 md:px-12">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-2xl font-bold text-blue-900 mb-6">🔥 Trending NFTs</h2>
          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-6">
            {/* Placeholder cards */}
            {Array(4).fill(0).map((_, i) => (
              <div
                key={i}
                className="bg-blue-50 p-4 rounded-xl shadow hover:shadow-lg transition"
              >
                <div className="h-40 bg-blue-100 rounded mb-4 animate-pulse"></div>
                <h3 className="font-semibold text-blue-900">NFT Title</h3>
                <p className="text-sm text-blue-600">Creator Name</p>
              </div>
            ))}
          </div>
        </div>
      </section>
    </div>
  );
};

export default Home;
