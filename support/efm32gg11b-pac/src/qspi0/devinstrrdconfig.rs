#[doc = "Register `DEVINSTRRDCONFIG` reader"]
pub type R = crate::R<DEVINSTRRDCONFIG_SPEC>;
#[doc = "Register `DEVINSTRRDCONFIG` writer"]
pub type W = crate::W<DEVINSTRRDCONFIG_SPEC>;
#[doc = "Field `RDOPCODENONXIP` reader - Read Opcode in Non-XIP Mode"]
pub type RDOPCODENONXIP_R = crate::FieldReader;
#[doc = "Field `RDOPCODENONXIP` writer - Read Opcode in Non-XIP Mode"]
pub type RDOPCODENONXIP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTRTYPE` reader - Instruction Type"]
pub type INSTRTYPE_R = crate::FieldReader;
#[doc = "Field `INSTRTYPE` writer - Instruction Type"]
pub type INSTRTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDREN` reader - DDR Enable"]
pub type DDREN_R = crate::BitReader;
#[doc = "Field `DDREN` writer - DDR Enable"]
pub type DDREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRXFERTYPESTDMODE` reader - Address Transfer Type for Standard SPI Modes"]
pub type ADDRXFERTYPESTDMODE_R = crate::FieldReader;
#[doc = "Field `ADDRXFERTYPESTDMODE` writer - Address Transfer Type for Standard SPI Modes"]
pub type ADDRXFERTYPESTDMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAXFERTYPEEXTMODE` reader - Data Transfer Type for Standard SPI Modes"]
pub type DATAXFERTYPEEXTMODE_R = crate::FieldReader;
#[doc = "Field `DATAXFERTYPEEXTMODE` writer - Data Transfer Type for Standard SPI Modes"]
pub type DATAXFERTYPEEXTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODEBITENABLE` reader - Mode Bit Enable"]
pub type MODEBITENABLE_R = crate::BitReader;
#[doc = "Field `MODEBITENABLE` writer - Mode Bit Enable"]
pub type MODEBITENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMYRDCLKCYCLES` reader - Dummy Read Clock Cycles"]
pub type DUMMYRDCLKCYCLES_R = crate::FieldReader;
#[doc = "Field `DUMMYRDCLKCYCLES` writer - Dummy Read Clock Cycles"]
pub type DUMMYRDCLKCYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&self) -> RDOPCODENONXIP_R {
        RDOPCODENONXIP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&self) -> INSTRTYPE_R {
        INSTRTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DDREN_R {
        DDREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&self) -> ADDRXFERTYPESTDMODE_R {
        ADDRXFERTYPESTDMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&self) -> DATAXFERTYPEEXTMODE_R {
        DATAXFERTYPEEXTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&self) -> MODEBITENABLE_R {
        MODEBITENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&self) -> DUMMYRDCLKCYCLES_R {
        DUMMYRDCLKCYCLES_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rdopcodenonxip(&mut self) -> RDOPCODENONXIP_W<DEVINSTRRDCONFIG_SPEC> {
        RDOPCODENONXIP_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    #[must_use]
    pub fn instrtype(&mut self) -> INSTRTYPE_W<DEVINSTRRDCONFIG_SPEC> {
        INSTRTYPE_W::new(self, 8)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddren(&mut self) -> DDREN_W<DEVINSTRRDCONFIG_SPEC> {
        DDREN_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    #[must_use]
    pub fn addrxfertypestdmode(&mut self) -> ADDRXFERTYPESTDMODE_W<DEVINSTRRDCONFIG_SPEC> {
        ADDRXFERTYPESTDMODE_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    #[must_use]
    pub fn dataxfertypeextmode(&mut self) -> DATAXFERTYPEEXTMODE_W<DEVINSTRRDCONFIG_SPEC> {
        DATAXFERTYPEEXTMODE_W::new(self, 16)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modebitenable(&mut self) -> MODEBITENABLE_W<DEVINSTRRDCONFIG_SPEC> {
        MODEBITENABLE_W::new(self, 20)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dummyrdclkcycles(&mut self) -> DUMMYRDCLKCYCLES_W<DEVINSTRRDCONFIG_SPEC> {
        DUMMYRDCLKCYCLES_W::new(self, 24)
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
#[doc = "Device Read Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devinstrrdconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devinstrrdconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVINSTRRDCONFIG_SPEC;
impl crate::RegisterSpec for DEVINSTRRDCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinstrrdconfig::R`](R) reader structure"]
impl crate::Readable for DEVINSTRRDCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devinstrrdconfig::W`](W) writer structure"]
impl crate::Writable for DEVINSTRRDCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVINSTRRDCONFIG to value 0x03"]
impl crate::Resettable for DEVINSTRRDCONFIG_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
