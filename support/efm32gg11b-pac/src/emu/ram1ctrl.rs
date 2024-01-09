#[doc = "Register `RAM1CTRL` reader"]
pub type R = crate::R<RAM1CTRL_SPEC>;
#[doc = "Register `RAM1CTRL` writer"]
pub type W = crate::W<RAM1CTRL_SPEC>;
#[doc = "Field `RAMPOWERDOWN` reader - RAM1 Blockset Power-down"]
pub type RAMPOWERDOWN_R = crate::FieldReader<RAMPOWERDOWN_A>;
#[doc = "RAM1 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "128: Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    BLK7 = 128,
    #[doc = "192: Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    BLK6TO7 = 192,
    #[doc = "224: Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    BLK5TO7 = 224,
    #[doc = "240: Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    BLK4TO7 = 240,
    #[doc = "248: Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    BLK3TO7 = 248,
    #[doc = "252: Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    BLK2TO7 = 252,
    #[doc = "254: Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    BLK1TO7 = 254,
    #[doc = "255: Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    BLK0TO7 = 255,
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
            128 => Some(RAMPOWERDOWN_A::BLK7),
            192 => Some(RAMPOWERDOWN_A::BLK6TO7),
            224 => Some(RAMPOWERDOWN_A::BLK5TO7),
            240 => Some(RAMPOWERDOWN_A::BLK4TO7),
            248 => Some(RAMPOWERDOWN_A::BLK3TO7),
            252 => Some(RAMPOWERDOWN_A::BLK2TO7),
            254 => Some(RAMPOWERDOWN_A::BLK1TO7),
            255 => Some(RAMPOWERDOWN_A::BLK0TO7),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK7
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk6to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK6TO7
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk5to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK5TO7
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk4to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK4TO7
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk3to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3TO7
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk2to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO7
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk1to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO7
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn is_blk0to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO7
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM1 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, RAMPOWERDOWN_A>;
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
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK7)
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk6to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK6TO7)
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk5to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK5TO7)
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk4to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK4TO7)
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk3to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK3TO7)
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk2to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK2TO7)
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk1to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK1TO7)
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk0to7(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPOWERDOWN_A::BLK0TO7)
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W<RAM1CTRL_SPEC> {
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
#[doc = "Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM1CTRL_SPEC;
impl crate::RegisterSpec for RAM1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1ctrl::R`](R) reader structure"]
impl crate::Readable for RAM1CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ram1ctrl::W`](W) writer structure"]
impl crate::Writable for RAM1CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM1CTRL to value 0"]
impl crate::Resettable for RAM1CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
