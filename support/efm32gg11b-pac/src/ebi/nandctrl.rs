#[doc = "Register `NANDCTRL` reader"]
pub type R = crate::R<NANDCTRL_SPEC>;
#[doc = "Register `NANDCTRL` writer"]
pub type W = crate::W<NANDCTRL_SPEC>;
#[doc = "Field `EN` reader - NAND Flash Control Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - NAND Flash Control Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKSEL` reader - NAND Flash Bank"]
pub type BANKSEL_R = crate::FieldReader<BANKSEL_A>;
#[doc = "NAND Flash Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKSEL_A {
    #[doc = "0: Memory bank 0 is connected to a NAND Flash device."]
    BANK0 = 0,
    #[doc = "1: Memory bank 1 is connected to a NAND Flash device."]
    BANK1 = 1,
    #[doc = "2: Memory bank 2 is connected to a NAND Flash device."]
    BANK2 = 2,
    #[doc = "3: Memory bank 3 is connected to a NAND Flash device."]
    BANK3 = 3,
}
impl From<BANKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BANKSEL_A {
    type Ux = u8;
}
impl BANKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BANKSEL_A {
        match self.bits {
            0 => BANKSEL_A::BANK0,
            1 => BANKSEL_A::BANK1,
            2 => BANKSEL_A::BANK2,
            3 => BANKSEL_A::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSEL_A::BANK0
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSEL_A::BANK1
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSEL_A::BANK2
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSEL_A::BANK3
    }
}
#[doc = "Field `BANKSEL` writer - NAND Flash Bank"]
pub type BANKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BANKSEL_A>;
impl<'a, REG> BANKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK0)
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK1)
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK2)
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK3)
    }
}
impl R {
    #[doc = "Bit 0 - NAND Flash Control Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BANKSEL_R {
        BANKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NAND Flash Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<NANDCTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline(always)]
    #[must_use]
    pub fn banksel(&mut self) -> BANKSEL_W<NANDCTRL_SPEC> {
        BANKSEL_W::new(self, 4)
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
#[doc = "NAND Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nandctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nandctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NANDCTRL_SPEC;
impl crate::RegisterSpec for NANDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nandctrl::R`](R) reader structure"]
impl crate::Readable for NANDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nandctrl::W`](W) writer structure"]
impl crate::Writable for NANDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NANDCTRL to value 0"]
impl crate::Resettable for NANDCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
