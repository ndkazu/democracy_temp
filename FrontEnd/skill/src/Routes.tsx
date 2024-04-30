import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import App from './App';
import Dashboard from './components/pages/Dashboard';
import Employee from './components/pages/Employee';
import Market from './components/pages/Market';
import Council from './components/pages/Council';

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    children: [
      { index: true, path: 'dashboard', element: <Dashboard /> },
      { path: 'employee', element: <Employee /> },
      { path: 'market', element: <Market /> },
      { path: 'council', element: <Council /> },
    ],
  },
]);
export function Routes() {
  return <RouterProvider router={router} />;
}
