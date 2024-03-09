import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import App from './App';
import Dashboard from './components/pages/Dashboard';
import Employee from './components/pages/Employee';

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    children: [
      { index: true, path: 'dashboard', element: <Dashboard /> },
      { path: 'employee', element: <Employee /> },
    ],
  },
]);
export function Routes() {
  return <RouterProvider router={router} />;
}
