import React from 'react';
import { BrowserRouter } from 'react-router-dom';
import AppRoutes from './routes';
import { AppLayout } from './layouts/AppLayout';

function App(): JSX.Element {
  return (
    <React.StrictMode>
      <BrowserRouter>
        <AppLayout>
          <AppRoutes />
        </AppLayout>
      </BrowserRouter>
    </React.StrictMode>
  );
}

export default App;
