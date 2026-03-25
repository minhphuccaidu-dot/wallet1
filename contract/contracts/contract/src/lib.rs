#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token};

#[contract]
pub struct FreelancerEscrowContract;

#[contractimpl]
impl FreelancerEscrowContract {
    
    /// BƯỚC 1: Khách hàng (Client) nạp tiền (VD: USDC) vào hợp đồng Ký quỹ
    /// client: Địa chỉ ví của khách hàng
    /// token_address: Địa chỉ của token (VD: contract ID của USDC)
    /// amount: Số lượng tiền cần khóa lại
    pub fn deposit(env: Env, client: Address, token_address: Address, amount: i128) {
        // Yêu cầu chữ ký xác thực từ ví của khách hàng
        client.require_auth();
        
        // Khởi tạo interface của Token
        let token = token::Client::new(&env, &token_address);
        
        // Chuyển tiền từ ví Client vào địa chỉ của chính Smart Contract này
        token.transfer(&client, &env.current_contract_address(), &amount);
    }

    /// BƯỚC 2: Khách hàng nghiệm thu và giải ngân tiền cho Freelancer
    /// client: Địa chỉ ví của khách hàng (người có quyền giải ngân)
    /// freelancer: Địa chỉ ví của người nhận tiền
    /// token_address: Địa chỉ của token đang được lưu giữ
    /// amount: Số tiền cần giải ngân
    pub fn release(env: Env, client: Address, freelancer: Address, token_address: Address, amount: i128) {
        // Chỉ khách hàng mới có quyền gọi lệnh giải ngân này
        client.require_auth(); 
        
        let token = token::Client::new(&env, &token_address);
        
        // Chuyển tiền từ Smart Contract sang ví của Freelancer
        token.transfer(&env.current_contract_address(), &freelancer, &amount);
    }
}
stellar contract invoke \
  --id CBP7H4BTHPMJLJSL3KIQFI4Z6UXGTI3BHWJTFLZRMTL7FIEGFS3YD237 \
  --source-account student \
  --network testnet \
  -- \
  deposit \
  --client "GCVAACK3JY6KWI22QKKYQLVOLYQKASWRWXGAOF5FTKXEGUYEBXF7X52E" \
  --token_address "CDLZFC3SYJYDZT7K67VZ75YJBMKBAV265DA76A5YZHO3Z6S2Z6G2677M" \
  --amount 10
  