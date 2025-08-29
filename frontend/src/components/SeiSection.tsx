import React from "react";

export const DuckchainSection: React.FC = () => {
  return (
    <section className="bg-white text-black py-20 px-6 md:px-12 lg:px-24">
      <div className="max-w-6xl mx-auto">
        <div className="flex flex-col lg:flex-row items-center">
          {/* Left side - Content */}
          <div className="lg:w-2/3 lg:pr-16">
            <h2 className="text-5xl md:text-7xl font-black uppercase mb-8">
              Built on
              <span className="text-duck-yellow"> Duckchain</span>
            </h2>

            <div className="space-y-6">
              <div className="flex items-center font-mono text-lg">
                <span className="text-duck-yellow mr-4">[✓]</span>
                <span className="font-black">TON ecosystem integration</span>
              </div>
              <div className="flex items-center font-mono text-lg">
                <span className="text-duck-yellow mr-4">[✓]</span>
                <span className="font-black">Telegram native connectivity</span>
              </div>
              <div className="flex items-center font-mono text-lg">
                <span className="text-duck-yellow mr-4">[✓]</span>
                <span className="font-black">
                  Consumer-focused applications
                </span>
              </div>
            </div>

            <div className="mt-12 border-4 border-black p-6">
              <p className="text-xl font-black uppercase">
                Mainstream blockchain adoption
              </p>
            </div>
          </div>

          {/* Right side - Geometric visual */}
          <div className="lg:w-1/3 mt-12 lg:mt-0">
            <div className="relative">
              <img src="/images/duckchainRocket.png" />
            </div>
          </div>
        </div>

        {/* Performance metrics */}
        <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8">
          <div className="text-center border-2 border-black p-8">
            <div className="font-mono text-4xl font-black text-duck-yellow mb-2">
              900M+
            </div>
            <div className="font-black uppercase text-lg">
              Telegram Users Access
            </div>
          </div>
          <div className="text-center border-2 border-black p-8">
            <div className="font-mono text-4xl font-black text-duck-yellow mb-2">
              Ultra-Low
            </div>
            <div className="font-black uppercase text-lg">Gas Fees</div>
          </div>
          <div className="text-center border-2 border-black p-8">
            <div className="font-mono text-4xl font-black text-duck-yellow mb-2">
              Instant
            </div>
            <div className="font-black uppercase text-lg">User Onboarding</div>
          </div>
        </div>
      </div>
    </section>
  );
};
