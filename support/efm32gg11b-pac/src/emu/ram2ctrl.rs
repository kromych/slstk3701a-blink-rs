#[doc = "Register `RAM2CTRL` reader"]
pub type R = crate::R<RAM2CTRL_SPEC>;
#[doc = "Register `RAM2CTRL` writer"]
pub type W = crate::W<RAM2CTRL_SPEC>;
#[doc = "Field `RAMPOWERDOWN` reader - RAM2 Blockset Power-down"]
pub type RAMPOWERDOWN_R = crate::FieldReader<RAMPOWERDOWN_A>;
#[doc = "RAM2 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "8: Power down RAM block 3"]
    BLK3 = 8,
    #[doc = "12: Power down RAM blocks 2-3"]
    BLK2TO3 = 12,
    #[doc = "14: Power down RAM blocks 1-3"]
    BLK1TO3 = 14,
    #[doc = "15: Power down RAM blocks 0-3"]
    BLK0TO3 = 15,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMPOWERDOWN_A {
    type Ux = u8;
}
impl RAMPOWERDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RAMPOWERDOWN_A> {
        match self.bits {
            0 => Some(RAMPOWERDOWN_A::NONE),
            8 => Some(RAMPOWERDOWN_A::BLK3),
            12 => Some(RAMPOWERDOWN_A::BLK2TO3),
            14 => Some(RAMPOWERDOWN_A::BLK1TO3),
            15 => Some(RAMPOWERDOWN_A::BLK0TO3),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Power down RAM block 3"]
    #[inline(always)]
    pub fn is_blk3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3
    }
    #[doc = "Power down RAM blocks 2-3"]
    #[inline(always)]
    pub fn is_blk2to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO3
    }
    #[doc = "Power down RAM blocks 1-3"]
    #[inline(always)]
    pub fn is_blk1to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO3
    }
    #[doc = "Power down RAM blocks 0-3"]
    #[inline(always)]
    pub fn is_blk0to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO3
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM2 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RAMPOWERDOWN_A>;
impl<'a, REG> RAMPOWERDOWN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 3"]
    #[inline(always)]
    pub fn blk3(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK3)
    }
    #[doc = "Power down RAM blocks 2-3"]
    #[inline(always)]
    pub fn blk2to3(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK2TO3)
    }
    #[doc = "Power down RAM blocks 1-3"]
    #[inline(always)]
    pub fn blk1to3(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK1TO3)
    }
    #[doc = "Power down RAM blocks 0-3"]
    #[inline(always)]
    pub fn blk0to3(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK0TO3)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM2 Blockset Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W<RAM2CTRL_SPEC> {
        RAMPOWERDOWN_W::new(self, 0)
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
#[doc = "Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM2CTRL_SPEC;
impl crate::RegisterSpec for RAM2CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram2ctrl::R`](R) reader structure"]
impl crate::Readable for RAM2CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ram2ctrl::W`](W) writer structure"]
impl crate::Writable for RAM2CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM2CTRL to value 0"]
impl crate::Resettable for RAM2CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
