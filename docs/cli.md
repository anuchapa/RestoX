### restox
--- for  cli part ---
## config currency {currency}

## user
# new {username} {password}
# change_password {username} {old_password} {new_password}

## login {username} {password}
## logout

## realtime (backgound service for notification)
# start
# end

## store
# create {name}
# delete -n{name}/-id{id}
# rename -n{name}/-id{id} {new_name}
# list

## staff
# add {name} {role}
# rename -n {old_name}/-id {id} {new_name}
# delete -n {name}/-id {id}
# change-role  {name} {new_role}

## table
# add {amount}
# delete {amount}

## menu
# add {name} {price} {ingredient_list}
# edit {name} {price} {ingredient_list}
# [optional] edit -n {name} -p {price} -i {ingredient_list} {amount_list} -ia(ingredient_add) {1 or more ingredient} {amount_list} -ir(ingredient_remove) {1 or more ingredient} {amount_list}(without this remove this from  this menu)
# list
# delete -n{name}/id{id}
# approve {transaction_id} {true or false} [optional if false]-c {comment}
# list wait approve

## ingredient
# add {name} {pirce} {unit}
# edit -n{old_name}/id{id} {new_name}
# [optional] edit n{old_name}/-id{id} -u {unit}
# fill -n{name}/-id{id} {amount} 
# list
# delete -n{name}/-id{id}
# approve {transaction_id} {true or false} [optional if false]-c {comment}
# list wait approve


### copilot review 11/08/2025
นี่คือรีวิวและข้อเสนอแนะสำหรับ draft CLI tree ของคุณในไฟล์ cli.md:

จุดแข็ง
ครอบคลุมฟังก์ชันหลักที่จำเป็นสำหรับระบบร้านอาหาร เช่น การจัดการร้าน, พนักงาน, โต๊ะ, เมนู, วัตถุดิบ, การแจ้งเตือน, การอนุมัติ ฯลฯ
โครงสร้างคำสั่งแบ่งกลุ่มชัดเจน (store, staff, table, menu, ingredient)
มีฟังก์ชันสำหรับการอนุมัติ (approve) และการแจ้งเตือน (realtime) ซึ่งเหมาะกับงานร้านอาหารจริง
ข้อเสนอแนะ
Consistency

ชื่อคำสั่งควรใช้รูปแบบเดียวกัน เช่น delete {name / id} บางที่ใช้ {name or id} ควรเลือกแบบเดียว
ingrediet ควรแก้เป็น ingredient
pirce ควรเป็น price
uint ควรเป็น unit
รายละเอียดเพิ่มเติม

คำสั่ง table add {amount} อาจระบุได้ว่าเพิ่มกี่โต๊ะ หรือเพิ่มโต๊ะหมายเลขอะไร
menu add และ edit ควรอธิบายรูปแบบ ingredient_list ให้ชัดเจน เช่น ใช้ comma-separated หรือ JSON
approve ควรระบุว่าใครมีสิทธิ์อนุมัติ (manager/kitchen)
ฟังก์ชันที่อาจขาด

ไม่มีคำสั่งเกี่ยวกับ order (เช่น รับออเดอร์, เปลี่ยนสถานะ, ดูคิว)
ไม่มีคำสั่งเกี่ยวกับ booking (จองโต๊ะ)
ไม่มีคำสั่งเกี่ยวกับ review หรือการให้คะแนน
ไม่มีคำสั่งเกี่ยวกับ payment หรือการคิดเงิน
อื่น ๆ

อาจเพิ่มตัวอย่างการใช้งานแต่ละคำสั่ง (usage example)
อาจเพิ่มคำสั่งสำหรับการ export/import ข้อมูล หรือ reset ระบบ
โดยรวม CLI tree นี้ครอบคลุมฟังก์ชันหลักของ environment ร้านอาหารได้ดี เหลือเพียงเติมรายละเอียดและเพิ่มบางฟีเจอร์ที่เกี่ยวกับ order, booking, review, payment เพื่อให้ครบถ้วนสมบูรณ์ยิ่งขึ้น หากต้องการให้ช่วยปรับโครงสร้างหรือเพิ่มตัวอย่าง แจ้งได้เลย!



