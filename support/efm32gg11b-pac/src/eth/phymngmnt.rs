#[doc = "Register `PHYMNGMNT` reader"]
pub type R = crate::R<PHYMNGMNT_SPEC>;
#[doc = "Register `PHYMNGMNT` writer"]
pub type W = crate::W<PHYMNGMNT_SPEC>;
#[doc = "Field `PHYRWDATA` reader - PHY read write data"]
pub type PHYRWDATA_R = crate::FieldReader<u16>;
#[doc = "Field `PHYRWDATA` writer - PHY read write data"]
pub type PHYRWDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE10` reader - Must be written with 10."]
pub type WRITE10_R = crate::FieldReader;
#[doc = "Field `WRITE10` writer - Must be written with 10."]
pub type WRITE10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGADDR` reader - Register address - specifies the register in the PHY to access."]
pub type REGADDR_R = crate::FieldReader;
#[doc = "Field `REGADDR` writer - Register address - specifies the register in the PHY to access."]
pub type REGADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHYADDR` reader - PHY address."]
pub type PHYADDR_R = crate::FieldReader;
#[doc = "Field `PHYADDR` writer - PHY address."]
pub type PHYADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OPERATION` reader - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OPERATION_R = crate::FieldReader;
#[doc = "Field `OPERATION` writer - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OPERATION_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE1` reader - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type WRITE1_R = crate::BitReader;
#[doc = "Field `WRITE1` writer - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type WRITE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE0` reader - Must be written with 0."]
pub type WRITE0_R = crate::BitReader;
#[doc = "Field `WRITE0` writer - Must be written with 0."]
pub type WRITE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&self) -> PHYRWDATA_R {
        PHYRWDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&self) -> WRITE10_R {
        WRITE10_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&self) -> WRITE1_R {
        WRITE1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&self) -> WRITE0_R {
        WRITE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    #[must_use]
    pub fn phyrwdata(&mut self) -> PHYRWDATA_W<PHYMNGMNT_SPEC> {
        PHYRWDATA_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    #[must_use]
    pub fn write10(&mut self) -> WRITE10_W<PHYMNGMNT_SPEC> {
        WRITE10_W::new(self, 16)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    #[must_use]
    pub fn regaddr(&mut self) -> REGADDR_W<PHYMNGMNT_SPEC> {
        REGADDR_W::new(self, 18)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    #[must_use]
    pub fn phyaddr(&mut self) -> PHYADDR_W<PHYMNGMNT_SPEC> {
        PHYADDR_W::new(self, 23)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    #[must_use]
    pub fn operation(&mut self) -> OPERATION_W<PHYMNGMNT_SPEC> {
        OPERATION_W::new(self, 28)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    #[must_use]
    pub fn write1(&mut self) -> WRITE1_W<PHYMNGMNT_SPEC> {
        WRITE1_W::new(self, 30)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    #[must_use]
    pub fn write0(&mut self) -> WRITE0_W<PHYMNGMNT_SPEC> {
        WRITE0_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PHY management register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phymngmnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phymngmnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHYMNGMNT_SPEC;
impl crate::RegisterSpec for PHYMNGMNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phymngmnt::R`](R) reader structure"]
impl crate::Readable for PHYMNGMNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phymngmnt::W`](W) writer structure"]
impl crate::Writable for PHYMNGMNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHYMNGMNT to value 0"]
impl crate::Resettable for PHYMNGMNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
