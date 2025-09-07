-- Clear existing data first
DELETE FROM payments;
DELETE FROM expenses;  
DELETE FROM invoices;
DELETE FROM products;
DELETE FROM suppliers;
DELETE FROM clients;

-- Seed sample clients
INSERT INTO clients (id, name, email, phone, address, tin, client_type, created_at, updated_at) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'ABC Corporation', 'contact@abccorp.ph', '+63 2 8888-1234', '123 Ayala Ave, Makati City', '123-456-789', 'business', datetime('now'), datetime('now')),
('550e8400-e29b-41d4-a716-446655440002', 'Juan Dela Cruz', 'juan@email.com', '+63 917 123-4567', '456 Rizal St, Quezon City', '987-654-321', 'individual', datetime('now'), datetime('now')),
('550e8400-e29b-41d4-a716-446655440003', 'Maria Santos', 'maria.santos@gmail.com', '+63 918 765-4321', '789 Bonifacio St, Taguig', '456-789-123', 'individual', datetime('now'), datetime('now')),
('550e8400-e29b-41d4-a716-446655440004', 'XYZ Enterprises', 'info@xyzent.ph', '+63 2 7777-5555', '321 BGC Tower, Taguig City', '789-123-456', 'business', datetime('now'), datetime('now')),
('550e8400-e29b-41d4-a716-446655440005', 'Tech Solutions Inc', 'sales@techsolutions.ph', '+63 2 9999-8888', '555 McKinley Rd, BGC', '321-654-987', 'business', datetime('now'), datetime('now'));

-- Seed sample suppliers
INSERT INTO suppliers (id, name, email, phone, address, tin, created_at, updated_at) VALUES
('660e8400-e29b-41d4-a716-446655440001', 'Office Supplies Co', 'orders@officesupplies.ph', '+63 2 5555-1111', '100 EDSA, Mandaluyong', '111-222-333', datetime('now'), datetime('now')),
('660e8400-e29b-41d4-a716-446655440002', 'Tech Hardware Store', 'sales@techhardware.ph', '+63 2 6666-2222', '200 Ortigas Ave, Pasig', '444-555-666', datetime('now'), datetime('now')),
('660e8400-e29b-41d4-a716-446655440003', 'Cleaning Services Ltd', 'info@cleaningservices.ph', '+63 2 7777-3333', '300 Shaw Blvd, Mandaluyong', '777-888-999', datetime('now'), datetime('now')),
('660e8400-e29b-41d4-a716-446655440004', 'Internet Provider Inc', 'support@internetprovider.ph', '+63 2 8888-4444', '400 Makati Ave, Makati', '111-444-777', datetime('now'), datetime('now'));

-- Seed sample products  
INSERT INTO products (id, name, description, unit_price, unit, vat_type, category, created_at, updated_at) VALUES
('770e8400-e29b-41d4-a716-446655440001', 'Laptop Computer', 'High-performance laptop for business use', 45000.00, 'unit', 'vatable', 'Electronics', datetime('now'), datetime('now')),
('770e8400-e29b-41d4-a716-446655440002', 'Office Chair', 'Ergonomic office chair with lumbar support', 8500.00, 'unit', 'vatable', 'Furniture', datetime('now'), datetime('now')),
('770e8400-e29b-41d4-a716-446655440003', 'Printer Paper', 'A4 size printer paper, 500 sheets per ream', 250.00, 'ream', 'vatable', 'Office Supplies', datetime('now'), datetime('now')),
('770e8400-e29b-41d4-a716-446655440004', 'Consulting Service', 'Professional consulting services per hour', 5000.00, 'hour', 'vatable', 'Services', datetime('now'), datetime('now')),
('770e8400-e29b-41d4-a716-446655440005', 'Web Development', 'Custom web application development', 150000.00, 'project', 'vatable', 'Services', datetime('now'), datetime('now')),
('770e8400-e29b-41d4-a716-446655440006', 'Rice', 'Premium white rice', 2500.00, '25kg', 'vat_exempt', 'Food', datetime('now'), datetime('now')),
('770e8400-e29b-41d4-a716-446655440007', 'Fresh Vegetables', 'Assorted fresh vegetables', 150.00, 'kg', 'vat_exempt', 'Food', datetime('now'), datetime('now'));

-- Seed sample invoices
INSERT INTO invoices (id, invoice_number, client_id, amount, vat_amount, total_amount, status, due_date, invoice_date, created_at, updated_at) VALUES
('880e8400-e29b-41d4-a716-446655440001', 'INV-2024-001', '550e8400-e29b-41d4-a716-446655440001', 100000.00, 12000.00, 112000.00, 'paid', '2024-02-15', '2024-01-15', datetime('now'), datetime('now')),
('880e8400-e29b-41d4-a716-446655440002', 'INV-2024-002', '550e8400-e29b-41d4-a716-446655440002', 45000.00, 5400.00, 50400.00, 'pending', '2024-03-01', '2024-02-01', datetime('now'), datetime('now')),
('880e8400-e29b-41d4-a716-446655440003', 'INV-2024-003', '550e8400-e29b-41d4-a716-446655440003', 25000.00, 3000.00, 28000.00, 'overdue', '2024-03-15', '2024-02-15', datetime('now'), datetime('now'));

-- Seed sample payments
INSERT INTO payments (id, payment_number, amount, payment_date, payment_method, reference_number, client_id, invoice_id, notes, created_at, updated_at) VALUES
('990e8400-e29b-41d4-a716-446655440001', 'PAY-2024-001', 112000.00, '2024-02-10', 'bank_transfer', 'BT-123456', '550e8400-e29b-41d4-a716-446655440001', '880e8400-e29b-41d4-a716-446655440001', 'Full payment received', datetime('now'), datetime('now')),
('990e8400-e29b-41d4-a716-446655440002', 'PAY-2024-002', 25000.00, '2024-02-20', 'cash', NULL, '550e8400-e29b-41d4-a716-446655440002', '880e8400-e29b-41d4-a716-446655440002', 'Partial payment', datetime('now'), datetime('now'));

-- Seed sample expenses
INSERT INTO expenses (id, expense_number, supplier_id, description, amount, vat_amount, total_amount, expense_date, category, created_at, updated_at) VALUES
('aa0e8400-e29b-41d4-a716-446655440001', 'EXP-2024-001', '660e8400-e29b-41d4-a716-446655440001', 'Monthly office supplies', 10000.00, 1200.00, 11200.00, '2024-01-10', 'Office Supplies', datetime('now'), datetime('now')),
('aa0e8400-e29b-41d4-a716-446655440002', 'EXP-2024-002', '660e8400-e29b-41d4-a716-446655440004', 'Internet subscription', 2500.00, 300.00, 2800.00, '2024-02-01', 'Utilities', datetime('now'), datetime('now'));