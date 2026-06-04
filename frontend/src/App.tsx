import { ThemeProvider } from './theme/ThemeContext';
import { ConverterPage } from './pages/ConverterPage';

export default function App() {
  return (
    <ThemeProvider initial="studio">
      <ConverterPage />
    </ThemeProvider>
  );
}
