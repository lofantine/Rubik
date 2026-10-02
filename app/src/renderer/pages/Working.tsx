import { useNavigate } from "react-router";

export default function Working() {
  const navigate = useNavigate();

  return (
    <main>
      <h1>Working</h1>
      <button onClick={() => navigate("/")}>Home</button>
    </main>
  );
}
