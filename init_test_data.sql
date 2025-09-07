-- Create clients table if not exists
CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    tin TEXT,
    client_type TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert test clients
INSERT OR REPLACE INTO clients (id, name, email, phone, address, tin, client_type) VALUES
('c1', 'Juan Dela Cruz', 'juan@example.com', '09171234567', '123 Main St, Manila', '123-456-789', 'individual'),
('c2', 'Maria Santos', 'maria@example.com', '09181234567', '456 Oak Ave, Quezon City', '987-654-321', 'individual'),
('c3', 'ABC Corporation', 'info@abc.com', '02-1234567', '789 Business Center, Makati', '111-222-333', 'corporate'),
('c4', 'XYZ Trading', 'contact@xyz.com', '02-7654321', '321 Commerce St, Pasig', '444-555-666', 'corporate'),
('c5', 'Pedro Reyes', 'pedro@example.com', '09191234567', '789 Elm St, Cebu', '777-888-999', 'individual');

-- Create products table if not exists
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    unit_price REAL,
    unit TEXT,
    vat_type TEXT,
    category TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert test products
INSERT OR REPLACE INTO products (id, name, description, unit_price, unit, vat_type, category) VALUES
('p1', 'Laptop Computer', 'High-performance laptop', 45000.00, 'unit', 'vatable', 'electronics'),
('p2', 'Office Chair', 'Ergonomic office chair', 5500.00, 'unit', 'vatable', 'furniture'),
('p3', 'Consultation Service', 'Professional consultation', 2500.00, 'hour', 'vatable', 'service'),
('p4', 'Rice', 'Premium rice', 60.00, 'kg', 'vat-exempt', 'food'),
('p5', 'Software License', 'Annual software license', 12000.00, 'year', 'vatable', 'software');

-- Create invoices table if not exists
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT NOT NULL UNIQUE,
    client_id TEXT,
    invoice_date DATE,
    due_date DATE,
    amount REAL,
    vat_amount REAL,
    total_amount REAL,
    status TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

-- Insert test invoices
INSERT OR REPLACE INTO invoices (id, invoice_number, client_id, invoice_date, due_date, amount, vat_amount, total_amount, status) VALUES
('inv1', 'INV-2024-001', 'c1', '2024-01-15', '2024-02-15', 10000.00, 1200.00, 11200.00, 'paid'),
('inv2', 'INV-2024-002', 'c2', '2024-02-01', '2024-03-01', 25000.00, 3000.00, 28000.00, 'unpaid'),
('inv3', 'INV-2024-003', 'c3', '2024-02-15', '2024-03-15', 50000.00, 6000.00, 56000.00, 'unpaid');

-- Create payments table if not exists
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    payment_number TEXT NOT NULL UNIQUE,
    client_id TEXT,
    invoice_id TEXT,
    payment_date DATE,
    amount REAL,
    payment_method TEXT,
    reference_number TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id),
    FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

-- Insert test payments
INSERT OR REPLACE INTO payments (id, payment_number, client_id, invoice_id, payment_date, amount, payment_method, reference_number) VALUES
('pay1', 'PAY-2024-001', 'c1', 'inv1', '2024-01-20', 11200.00, 'bank_transfer', 'REF123456'),
('pay2', 'PAY-2024-002', 'c3', 'inv3', '2024-02-20', 20000.00, 'check', 'CHK789012');