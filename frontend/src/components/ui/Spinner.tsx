export function Spinner() {
  return (
    <svg
      width="11"
      height="11"
      viewBox="0 0 11 11"
      fill="none"
      style={{ animation: 'spin 0.8s linear infinite' }}
    >
      <circle cx="5.5" cy="5.5" r="4" stroke="rgba(255,255,255,0.35)" strokeWidth="1.5" />
      <path
        d="M5.5 1.5A4 4 0 0 1 9.5 5.5"
        stroke="white"
        strokeWidth="1.5"
        strokeLinecap="round"
      />
    </svg>
  );
}
