import { DebugContracts } from "./_components/DebugContracts";
import type { NextPage } from "next";
import { getMetadata } from "~~/utils/scaffold-eth/getMetadata";
import { DebugAgeVerifier } from "./_components/AgeVerifier";


export const metadata = getMetadata({
  title: "Debug Contracts",
  description: "Debug your deployed 🏗 Scaffold-ETH 2 contracts in an easy way",
});

const Debug: NextPage = () => {
  return (
    <>
      {/* <DebugContracts /> */}
      <DebugAgeVerifier />
    </>
  );
};

export default Debug;
