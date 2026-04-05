#[doc = "Register `PHYMNGMNT` reader"]
pub type R = crate::R<PhymngmntSpec>;
#[doc = "Register `PHYMNGMNT` writer"]
pub type W = crate::W<PhymngmntSpec>;
#[doc = "Field `PHYRWDATA` reader - PHY read write data"]
pub type PhyrwdataR = crate::FieldReader<u16>;
#[doc = "Field `PHYRWDATA` writer - PHY read write data"]
pub type PhyrwdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE10` reader - Must be written with 10."]
pub type Write10R = crate::FieldReader;
#[doc = "Field `WRITE10` writer - Must be written with 10."]
pub type Write10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGADDR` reader - Register address - specifies the register in the PHY to access."]
pub type RegaddrR = crate::FieldReader;
#[doc = "Field `REGADDR` writer - Register address - specifies the register in the PHY to access."]
pub type RegaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHYADDR` reader - PHY address."]
pub type PhyaddrR = crate::FieldReader;
#[doc = "Field `PHYADDR` writer - PHY address."]
pub type PhyaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OPERATION` reader - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OperationR = crate::FieldReader;
#[doc = "Field `OPERATION` writer - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OperationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE1` reader - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type Write1R = crate::BitReader;
#[doc = "Field `WRITE1` writer - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type Write1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE0` reader - Must be written with 0."]
pub type Write0R = crate::BitReader;
#[doc = "Field `WRITE0` writer - Must be written with 0."]
pub type Write0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&self) -> PhyrwdataR {
        PhyrwdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&self) -> Write10R {
        Write10R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&self) -> RegaddrR {
        RegaddrR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PhyaddrR {
        PhyaddrR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&self) -> OperationR {
        OperationR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&self) -> Write1R {
        Write1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&self) -> Write0R {
        Write0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&mut self) -> PhyrwdataW<'_, PhymngmntSpec> {
        PhyrwdataW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&mut self) -> Write10W<'_, PhymngmntSpec> {
        Write10W::new(self, 16)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&mut self) -> RegaddrW<'_, PhymngmntSpec> {
        RegaddrW::new(self, 18)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&mut self) -> PhyaddrW<'_, PhymngmntSpec> {
        PhyaddrW::new(self, 23)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&mut self) -> OperationW<'_, PhymngmntSpec> {
        OperationW::new(self, 28)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&mut self) -> Write1W<'_, PhymngmntSpec> {
        Write1W::new(self, 30)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&mut self) -> Write0W<'_, PhymngmntSpec> {
        Write0W::new(self, 31)
    }
}
#[doc = "PHY management register\n\nYou can [`read`](crate::Reg::read) this register and get [`phymngmnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phymngmnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhymngmntSpec;
impl crate::RegisterSpec for PhymngmntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phymngmnt::R`](R) reader structure"]
impl crate::Readable for PhymngmntSpec {}
#[doc = "`write(|w| ..)` method takes [`phymngmnt::W`](W) writer structure"]
impl crate::Writable for PhymngmntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHYMNGMNT to value 0"]
impl crate::Resettable for PhymngmntSpec {}
