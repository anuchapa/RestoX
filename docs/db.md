# Database Design (ERD)

## Main table 

### stores/
- id PK
- name VARCHAR(255)


### tables/
- id PK
- store_id FK
- number NUMURICAL
- status (use, ready, inactive)


### table_session/
- id PK
- table_id FK
- booking_id FK
- time_start DATETIME
- time_end DATETIME

### users/
- id PK
- username VARCHAR(255)
- time_start DATETIME
- password_hash TEXT
- password_salt TEXT
- role (customer, kitchen, store)

### user_staffs (option)
### user_customers (option)


### menu/
- id PK
- store_id FK
- name VARCHAR(255)
- price NUMURICAL
- currency VARCHAR(5)

### ingredient_in_store/
- id PK
- store_id FK
- price_per_unit NUMURICAL
- currency ARCHAR(5)

### ingredient_stock/
- id PK
- ingredient_id FK
- balance NUMURICAL
- balance_unit VARCHAR(5)

### menu_ingredient_item/
- id PK
- menu_id FK
- ingredient_id FK
- change NUMURICAL


### menu_proposal/
- id PK
- store_id FK
- name VARCHAR(255)
- cost NUMURICAL
- currency VARCHAR(5)
- status (approve, reject)
- remark TEXT

### menu_proposal_ingredient_items/
- id PK
- menu_id FK
- ingredient_id FK
- change NUMURICAL

### ingredient_request/
- id PK
- store_id FK
- ingredient_id FK
- amount NUMURICAL
- request_type (add, remove, fill)
- status (wait_approve, filled)

### orders/
- id PK
- table_session_id FK
- status (fiished, unfinished)
- total NUMURICAL 
- currency VARCHAR(5)

### order_items/
- id PK
- menu_id
- orders_id FK
- quantity NUMURICAL
- price_per_unit NUMURICAL
- status (recieve_oder, cooking, finished, served)


### booking/
- id PK
- user_id FK
- store FK
- table FK
- booking_time DATETIME


### payment/
- id PK
- table_session_id FK
- total NUMURICAL
- currency VARCHAR(5)

### payment_items/
- id PK
- payment_id FK
- payment_name VARCAR(255)
- amount NUMURICAL
- status (paid, unpaid)

### review
- id PK
- table_sessin_id FK
- comment TEXT
- time DATETIME



### ai Review by  chat-gpt

# 2025/07/10
- โครงสร้างของคุณแน่นมากแล้ว — ครบเกือบทุกมุมของระบบร้านอาหารแบบยืดหยุ่น 👏
- menu_id ไม่ควรอยู่ที่ orders ย้ายไปที่ orders item 
- review 
    * table_session_id FK → โอเค
    * อาจเพิ่ม rating (1–5) ถ้าต้องการประเมินคุณภาพ
- users
    * แนะนำให้ใช้ ENUM สำหรับ role หรือ normalize เป็น user_roles table
- menu_ingredient_item vs menu_ingredient_proposal_item
    * ชัดเจนดี
    * อาจเปลี่ยนชื่อเป็น menu_ingredient_items และ menu_proposal_ingredient_items (ชื่อให้สอดคล้อง)

## Summary การปรับ:
| สิ่งที่ควรทำ                                             | เหตุผล                                           |
| --------------------------------------------------- | ---------------------------------------------- |
| สร้างตารางสถานะ (status tables)                        | ลด duplication, scale ง่าย, maintain ง่าย           |    
| ย้าย `menu_id` ออกจาก `orders`                        | เพราะควรอยู่ใน `order_items` แทน               |
| เพิ่ม `quantity`, `price_per_unit` ใน `order_items`    | เพื่อการคิดเงินและรายงาน                       |
| Normalize `currency` และ `roles`                     | เพื่อรองรับหลายภาษา หลายค่าเงิน หลาย user-type |
| Normalize `currency` และ `roles`                     | เพื่อรองรับหลายภาษา หลายค่าเงิน หลาย user-type |



# 2025/07/11 Review by GitHub Copilot

## ข้อเสนอแนะเพิ่มเติม

1. **สะกดคำผิด**
   - NUMURICAL → NUMERICAL
   - VARCAR/ARCHAR → VARCHAR
   - table_sessin_id → table_session_id
   - recieve_oder → receive_order
   - opion → option

2. **Consistency & Naming**
   - ใช้ชื่อฟิลด์และชื่อตารางให้สอดคล้อง เช่น menu_ingredient_item, menu_proposal_ingredient_items
   - ชื่อฟิลด์ FK ควรเติม _id ต่อท้ายเสมอ เช่น store_id, table_id

3. **รายละเอียดชนิดข้อมูล**
   - ระบุขนาด VARCHAR ให้ครบถ้วน
   - กำหนด ENUM หรือสร้าง status table สำหรับฟิลด์ที่มีค่าคงที่ เช่น status, role, currency

4. **Normalization**
   - พิจารณาแยกตาราง roles, currencies, status ออกจากตารางหลัก เพื่อความยืดหยุ่นและรองรับการขยายระบบ

5. **ความสัมพันธ์**
   - ตรวจสอบ FK ให้ถูกต้อง เช่น table_session → table_id, booking_id
   - เพิ่มคำอธิบายความสัมพันธ์ระหว่างตาราง (Relationship Diagram หรือ ERD)

6. **อื่น ๆ**
   - เพิ่มฟิลด์ rating ใน review หากต้องการประเมินคุณภาพ
   - เพิ่มคำอธิบายแต่ละตาราง/ฟิลด์ เพื่อความเข้าใจที่ดีขึ้น

---
ข้อดี: โครงสร้างโดยรวมครอบคลุมฟีเจอร์หลักของระบบร้านอาหาร มีความยืดหยุ่นและรองรับการขยายในอนาคตได้ดี
ข้อควรปรับ: ปรับปรุงเรื่อง consistency, การสะกด, normalization และรายละเอียดชนิดข้อมูล เพื่อให้ฐานข้อมูลมีความสมบูรณ์และดูแลรักษาง่าย
