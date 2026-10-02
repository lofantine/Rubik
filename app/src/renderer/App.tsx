import { Route, Routes } from "react-router";
import Home from "./pages/Home";
import Working from "./pages/Working";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/working" element={<Working />} />
    </Routes>
  );
}
