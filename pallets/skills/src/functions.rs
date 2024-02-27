pub use super::*;

impl<T: Config> Pallet<T> {

    //Helper function for skill approval
    pub fn approve_skill_helper(from_who:T::AccountId) -> DispatchResultWithPostInfo{
        let skill0 = SkillsApprovalList::<T>::iter();
        let mut exist = false;

        for skill in skill0{
            //Check that account is an employee 
            if from_who==skill.0{
                let mut sk = skill.1.clone();
                sk.skill_number=Self::skills().into_inner().len() as u8;

                //Add skill to skill database
                Skills::<T>::mutate(|list|{
                    list.try_push(sk.clone()).map_err(|_| "Max number of skills reached").ok();
                });

                //Remove skill from waiting list
                SkillsApprovalList::<T>::remove(&from_who);
                
                exist = true;
                break;
            }
        }
        ensure!(exist,Error::<T>::NoSkillSubmited);
        Ok(().into())
    }   



    //Helper function for skill approval
    pub fn reject_skill_helper(from_who:T::AccountId) -> DispatchResultWithPostInfo{
        let skills = SkillsApprovalList::<T>::iter();
        
        let mut exist = false;

        for skill in skills{
            //Check that account is an employee 
            if from_who==skill.0{

                //Remove skill from waiting list
                SkillsApprovalList::<T>::remove(&from_who);
                
                exist = true;
                break;
            }
        }
        ensure!(exist,Error::<T>::NoSkillSubmited);
        Ok(().into())
    }   

}